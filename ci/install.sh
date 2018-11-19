set -euxo pipefail

main() {
    case $TARGET in
        msp430-none-elf)
            cargo install --list | grep xargo || \
                cargo install xargo

            rustup component add rust-src

            local file=msp430-gcc-7.3.2.154_linux64.7z
            curl -LO http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/latest/exports/$file
            7za x $file
            ;;
    esac
}

main
