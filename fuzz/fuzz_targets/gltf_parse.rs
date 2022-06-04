#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(gltf) = gltf::Gltf::from_slice(data) {
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                println!(
                    "Node #{} has {} children",
                    node.index(),
                    node.children().count(),
                );
            }
        }
    }
});
