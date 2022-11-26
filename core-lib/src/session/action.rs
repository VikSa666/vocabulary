pub enum Action {
    AddWord {
        word: String,
        translations: String,
        set: String,
    },
    DeleteWord {
        word: String,
        set: String,
    },
    UpdateWordFiles,
}
