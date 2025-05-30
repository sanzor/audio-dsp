use crate::crud_dispatchers::*;
use crate::dispatcher_enum::DispatcherEnum;
use crate::dsp_dispatchers::*;
use crate::player_dispatchers::PlayDispatcher;
use std::collections::HashMap;

pub(crate) struct DispatchersProvider {
    pub dispatch_map: HashMap<String, DispatcherEnum>,
}

impl DispatchersProvider {
    pub fn new() -> DispatchersProvider {
        DispatchersProvider {
            dispatch_map: Self::create_dispatch_map(),
        }
    }
    fn create_dispatch_map() -> HashMap<String, DispatcherEnum> {
        let mut hash_map: HashMap<String, DispatcherEnum> = HashMap::new();
        DispatchersProvider::populate_dispatchers(&mut hash_map);
        hash_map
    }

    fn populate_dispatchers(hash_map: &mut HashMap<String, DispatcherEnum>) {
        hash_map.insert("load".to_string(), DispatcherEnum::Load(LoadDispatcher {}));
        hash_map.insert("copy".to_string(), DispatcherEnum::Copy(CopyDispatcher {}));
        hash_map.insert("ls".to_string(), DispatcherEnum::List(ListDispatcher {}));
        hash_map.insert(
            "delete".to_string(),
            DispatcherEnum::Delete(DeleteDispatcher {}),
        );
        hash_map.insert("info".to_string(), DispatcherEnum::Info(InfoDispatcher {}));
        hash_map.insert(
            "upload".to_string(),
            DispatcherEnum::Upload(UploadDispatcher {}),
        );
        hash_map.insert("gain".to_string(), DispatcherEnum::Gain(GainDispatcher {}));
        hash_map.insert(
            "normalize".to_string(),
            DispatcherEnum::Normalize(NormalizeDispatcher {}),
        );
        hash_map.insert(
            "low_pass".to_string(),
            DispatcherEnum::LowPass(LowPassDispatcher {}),
        );
        hash_map.insert(
            "high_pass".to_string(),
            DispatcherEnum::HighPass(HighPassDispatcher {}),
        );
        hash_map.insert(
            "run-script".to_string(),
            DispatcherEnum::RunScript(RunScriptDispatcher {}),
        );
        hash_map.insert("play".to_string(), DispatcherEnum::Play(PlayDispatcher {}));
        hash_map.insert(
            "pause".to_string(),
            DispatcherEnum::RunScript(RunScriptDispatcher {}),
        );
    }
    pub fn get_dispatcher_by_name(&self, name: &str) -> Option<&DispatcherEnum> {
        self.dispatch_map.get(name)
    }
}
