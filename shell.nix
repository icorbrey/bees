{ pkgs, ... }: {
  default = pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      cargo
      openssl.dev
      pkg-config
      rustc
    ];
    
    buildInputs = with pkgs; [
      docker
      helm
      kind
      kubectl
      sqlx-cli
      tilt
    ];

    shellHook = ''
      raw_out="$(docker info 2>&1)" || {
        if echo "$raw_out" | grep -qi "permission denied"; then
          echo "🔒 Docker permission denied. Make sure you’re in the 'docker' group and have re-logged in (or run 'newgrp docker')."
        else
          echo "🔴 Docker daemon is not running. Please start Docker and try again."
        fi
        return 1
      }

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
