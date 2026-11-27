pub mod header;
pub mod home;
pub mod footer;
pub mod success_modal;

// Реэкспорт, чтобы удобно импортировать сразу компоненты
pub use header::Header;
pub use home::Home;
pub use footer::Footer;
pub use success_modal::SuccessModal;
