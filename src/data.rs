use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: " ".to_string(),
            price: 1599.99,
            description: "Experience cutting-edge performance with the iPhone 15 Pro Max. Titanium design, A17 Pro chip, and a pro camera system built for creative professionals.".to_string(),
            image: "/iphone15pro.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Samsung 65\" QLED 8K Smart TV".to_string(),
            price: 2499.99,
            description: "Enjoy lifelike clarity with Samsung's QLED 8K Smart TV. AI-powered upscaling and vibrant Quantum Dot display bring every scene to life.".to_string(),
            image: "/samsung_qled.jpg".to_string()
        },
        Product {
            id: 3,
            name: "ASUS ROG Strix Gaming Laptop".to_string(),
            price: 1999.99,
            description: "Crush the competition with the ASUS ROG Strix. Featuring an AMD Ryzen 9, RTX 4080 GPU, and a 240Hz FHD display for ultra-smooth gameplay.".to_string(),
            image: "/asus_rog.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Canon EOS R8 Mirrorless Camera".to_string(),
            price: 1899.99,
            description: "Capture professional-quality images with the Canon EOS R8. Compact, powerful, and ideal for both creators and enthusiasts.".to_string(),
            image: "/canon_eos.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Dyson V15 Detect Vacuum Cleaner".to_string(),
            price: 849.99,
            description: "The most powerful Dyson cordless vacuum with laser detection for hidden dust. Smart LCD screen shows real-time performance.".to_string(),
            image: "/dyson_v15.jpg".to_string()
        },
        Product {
            id: 6,
            name: "HP ENVY Desktop PC - Intel i7".to_string(),
            price: 1099.99,
            description: "A powerful home or office PC with 13th Gen Intel i7, 16GB RAM, and 1TB SSD. Sleek and quiet with ample ports.".to_string(),
            image: "/hp_envy.jpg".to_string()
        },
        Product {
            id: 7,
            name: "DJI Mini 4 Pro Drone".to_string(),
            price: 999.99,
            description: "Fly smarter with the DJI Mini 4 Pro. 4K video, obstacle sensing, and ultralight design make it perfect for content creators on the move.".to_string(),
            image: "/dji_mini.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Logitech MX Master 3S Wireless Mouse".to_string(),
            price: 139.99,
            description: "Ultimate precision and comfort with the Logitech MX Master 3S. Silent clicks, fast scrolling, and multi-device support.".to_string(),
            image: "/logitech_mx.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Ring Video Doorbell Pro 2".to_string(),
            price: 349.99,
            description: "Stay secure with the Ring Video Doorbell Pro 2. Features 3D Motion Detection, 1536p HD video, and head-to-toe viewing.".to_string(),
            image: "/ring_doorbell.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Philips Hue White & Color Ambiance Starter Kit".to_string(),
            price: 229.99,
            description: "Smart lighting that transforms your space. Includes 3 color bulbs and a hub compatible with Alexa, Google, and Apple HomeKit.".to_string(),
            image: "/philips_hue.jpg".to_string()
        },
        Product {
            id: 11,
            name: "Seagate 2TB Portable External HDD".to_string(),
            price: 79.99,
            description: "Back up files on the go with the Seagate 2TB portable hard drive. USB 3.0 interface ensures fast data transfer.".to_string(),
            image: "/seagate_2tb.jpg".to_string()
        },
        Product {
            id: 12,
            name: "JBL Flip 6 Portable Bluetooth Speaker".to_string(),
            price: 169.99,
            description: "Powerful sound, bold design. The JBL Flip 6 is waterproof, portable, and perfect for music lovers everywhere.".to_string(),
            image: "/jbl_flip6.jpg".to_string()
        }
        
    ]
}