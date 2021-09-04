pub struct SomethingDto<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub alias: &'a str,
    pub email: &'a str,
}
