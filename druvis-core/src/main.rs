fn main() {
    pollster::block_on(druvis_core::instance::instance::run());
}
