# From https://jordankaye.dev/posts/rust-wasm-nix/
{
  description = "Rust Cloudflare Latency Test Worker for Web of Song";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        f = with fenix.packages.${system};
          combine [
            stable.toolchain
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        vscodeCustomized = (pkgs.vscode-with-extensions.override {
          vscodeExtensions = with pkgs.vscode-extensions; [
            vscodevim.vim
            hashicorp.terraform
          ];
        });
        mkScript = text:
          let
            script = pkgs.writeShellApplication {
              inherit text;
              name = "script.sh";
            };
          in "${script}/bin/script.sh";
        mkScriptApp = text: {
          type = "app";
          program = mkScript text;
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "wos-latency-worker-shell";

          packages = with pkgs; [
            f
            linuxPackages_latest.perf
            lldb
            llvmPackages.bintools
            nodePackages.wrangler
            nodejs_21
            vscode-langservers-extracted
            terraform
            wasm-pack
          ];

          CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
        };

        apps = {
          deploy = mkScriptApp "wrangler deploy";
          tf-init = mkScriptApp ''
            cd terraform
            terraform init -var-file oliver.tfvars
          '';
          tf-apply = mkScriptApp ''
            cd terraform
            terraform apply -var-file oliver.tfvars
          '';
          tf-destroy = mkScriptApp ''
            cd terraform
            terraform destroy -var-file oliver.tfvars
          '';
        };
      });
}
