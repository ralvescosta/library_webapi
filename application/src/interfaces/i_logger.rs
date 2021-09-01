pub trait ILogger {
    fn trace(&self, msg: String);
    fn debug(&self, msg: String);
    fn info(&self, msg: String);
    fn warn(&self, msg: String);
    fn error(&self, msg: String);
}
