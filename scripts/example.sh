# Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.
set -o errexit
set -o nounset
set -o pipefail

echo ""
echo "--------------------------------"
echo "Select example to run: "
echo "--------------------------------"
echo ""

select opt in  csm ctx smoking quit;
do
  case $opt in
    csm)
      echo "Selected example: CSM (Causal State Machine)"
      command cargo run --release --example csm
      break
      ;;
    ctx)
      echo "Selected example: CTX (Context)"
      command cargo run --release --example ctx
      break
      ;;
    smoking)
      echo "Selected example: SMOKING (Smoking)"
      command cargo run --release --example smoking
      break
      ;;
    quit)
      echo "Exiting!"
      exit 0
      ;;
    *)
      echo "Invalid option $REPLY"
      ;;
  esac
done
