use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 65\" 4K QLED TV".to_string(),
            price: 1299.99,
            description: "Immerse yourself in stunning 4K resolution with Samsung's QLED technology. Features Quantum HDR, a 120Hz refresh rate, and built-in Alexa and Google Assistant for a smart home experience.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple MacBook Air M2".to_string(),
            price: 1099.99,
            description: "Supercharged by the Apple M2 chip, the MacBook Air delivers exceptional performance in an incredibly thin and light design. Features a 13.6\" Liquid Retina display and up to 18 hours of battery life.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 399.99,
            description: "Industry-leading noise cancellation with eight microphones and two processors. Enjoy up to 30 hours of battery life, crystal-clear hands-free calling, and multipoint connection for two devices.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 4,
            name: "Apple iPhone 15 Pro".to_string(),
            price: 1199.99,
            description: "Forged in titanium and featuring the groundbreaking A17 Pro chip, iPhone 15 Pro has a pro camera system with a 48MP main camera, Action Button, and USB 3 speeds.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 5,
            name: "PlayStation 5 Console".to_string(),
            price: 649.99,
            description: "Experience lightning-fast loading with an ultra-high-speed SSD, deeper immersion with haptic feedback and adaptive triggers, and stunning 4K gaming with ray tracing support.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 6,
            name: "Apple iPad Pro 12.9\"".to_string(),
            price: 1299.99,
            description: "The ultimate iPad experience with the Apple M2 chip, a stunning Liquid Retina XDR display, and support for Apple Pencil and the Magic Keyboard. Perfect for creative professionals.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 7,
            name: "Samsung Galaxy S24 Ultra".to_string(),
            price: 1399.99,
            description: "The most powerful Galaxy ever, featuring a built-in S Pen, 200MP camera, and Galaxy AI. The titanium frame and 6.8\" Dynamic AMOLED display make it a premium powerhouse.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 8,
            name: "Bose QuietComfort Ultra Earbuds".to_string(),
            price: 299.99,
            description: "Bose's best noise-cancelling earbuds ever. With CustomTune technology, they automatically personalize sound and noise cancellation to your ears for an unmatched listening experience.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 9,
            name: "Nintendo Switch OLED".to_string(),
            price: 349.99,
            description: "Play at home or on the go with the Nintendo Switch OLED model. Features a vibrant 7\" OLED screen, a wide adjustable stand, and 64GB of internal storage.".to_string(),
            image: "/placeholder.png".to_string()
        },
        Product {
            id: 10,
            name: "Dell 27\" 4K USB-C Monitor".to_string(),
            price: 549.99,
            description: "A stunning 27-inch 4K IPS display with USB-C connectivity for single-cable docking. Features 99% sRGB color coverage, built-in speakers, and adjustable stand for ergonomic comfort.".to_string(),
            image: "/placeholder.png".to_string()
        }
    ]
}