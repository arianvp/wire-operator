{
  description = "EKS Experiment";

  inputs.utils.url = "github:numtide/flake-utils";

  outputs = { self, utils, nixpkgs }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            openssl
          ];
          nativeBuildInputs = [

            pkgconfig

            kubectl
            kubernetes-helm
            kustomize
            terraform_0_13
            shellcheck
            kind
            jq

            rustc
            rls
            rust-analyzer
            cargo
          ];
        };
      });
}
