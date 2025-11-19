use axum::extract::WebSocketUpgrade;
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;

pub struct WsController {
    
}

impl WsController  {
    pub fn new() -> Self {
        Self {}
    }
    
    pub(crate) async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
        println!("🔄 WebSocket подключение запрошено");

        ws.on_upgrade(Self::handle_websocket)    
    }

    pub async fn handle_websocket( mut socket: WebSocket) {
        println!("✅ WebSocket клиент подключен");

        // Отправляем приветственное сообщение
        if let Err(e) = socket.send("Привет! Вы подключились к WebSocket".into()).await {
            eprintln!("❌ Ошибка отправки: {}", e);
            return;
        }

        // Простой цикл для получения сообщений
        while let Some(message) = socket.recv().await {
            match message {
                Ok(msg) => {
                    println!("📨 Получено сообщение: {:?}", msg);

                    // Отвечаем эхо-сообщением
                    if let Err(e) = socket.send(format!("Эхо: {:?}", msg).into()).await {
                        eprintln!("❌ Ошибка отправки: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("❌ Ошибка получения: {}", e);
                    break;
                }
            }
        }

        println!("🔌 WebSocket клиент отключен");
    }
}
