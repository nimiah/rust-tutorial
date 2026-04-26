use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Deserialize, Clone, Default, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
    #[serde(default="default_server_port")]
    pub server_port: String
}

fn default_server_port() -> String{
    "4000".to_string()
}

pub static CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn init()  {
        // LUẬT ƯU TIÊN: Load cái quan trọng nhất TRƯỚC.
        // Nếu dev.env có biến DATABASE_URL, nó sẽ được nạp vào môi trường.
        // Sau đó load .env, nếu thấy DATABASE_URL đã có rồi, nó sẽ bỏ qua.
        let _ = dotenvy::from_filename("dev.env"); // Ưu tiên 1 (Member-specific)
        let _ = dotenvy::dotenv(); // Ưu tiên 2 (Common config)

        // Dùng envy để map toàn bộ env thành Struct (Type-safe)
        // Nếu thiếu bất kỳ biến nào, chương trình sẽ crash ngay lập tức (Fail-fast)
        let config: AppConfig = envy::from_env::<AppConfig>()
            .expect("❌ Thiếu biến! Kiểm tra lại file .env hoặc dev.env nhé.");

        // Engineer Notice: Không bao giờ in SECRET_KEY ra log ở môi trường thật!
        // Chỉ nên in ở local để debug.
        #[cfg(debug_assertions)]
        {
            dbg!(&config);
        }

        // Lưu vào Global Static
        if CONFIG.set(config).is_err() {
            panic!("❌ CONFIG đã được khởi tạo trước đó rồi!")
        }

    }

    pub fn get() -> &'static AppConfig {
        CONFIG
            .get()
            .expect("❌ AppConfig chưa được khởi tạo! Hãy gọi AppConfig::init() trong main.rs")
    }
}
