{ pkgs, ... }: {
  default = pkgs.mkShell {
    buildInputs = with pkgs; [
      docker
      helm
      kind
      kubectl
      tilt
    ];

    shellHook = ''
      if ! docker info > /dev/null 2>&1; then
        echo "🔴 Docker daemon is not running. Please start Docker and try again."
        return 1
      fi

      if ! kind get clusters | grep -q "^dev$"; then
        echo "⚡ Creating a 'dev' kind cluster..."
        kind create cluster --name dev
      fi
      
      KUBECONFIG_FILE="$(mktemp --suffix=-kind-config)"
      kind get kubeconfig --name dev > "$KUBECONFIG_FILE"
      export KUBECONFIG="$KUBECONFIG_FILE"

      echo "⚡ Ready! kubectl context: $(kubectl config current-context)"
    '';
  };
}
