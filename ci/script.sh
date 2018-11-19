set -euxo pipefail

main() {
    case $TARGET in
        msp430-none-elf)
            xargo check --target $TARGET
            ;;
        *)
            cargo test --target $TARGET
            ;;
    esac
}

main
