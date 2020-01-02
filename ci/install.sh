set -euxo pipefail

main() {
    case $TARGET in
        msp430-none-elf)
            cargo install --list | grep xargo || \
                cargo install xargo

            rustup component add rust-src

            curl -LO http://software-dl.ti.com/msp430/msp430_public_sw/mcu/msp430/MSPGCC/8_3_1_0/export/msp430-gcc-8.3.1.25_linux64.tar.bz2
            tar xjf msp430-gcc-8.3.1.25_linux64.tar.bz2
            ;;
    esac
}

main
