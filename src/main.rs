use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_PIN: u8 = 18; // GPIO18

fn main() -> Result<(), Box<dyn Error>> {
    // Получаем доступ к GPIO
    let gpio = Gpio::new()?;

    // Настраиваем пин как выход
    let mut pin = gpio.get(GPIO_PIN)?.into_output();

    println!("Лазерный модуль запущен. Нажмите Ctrl+C для выхода.");

    // Бесконечный цикл
    loop {
        // Включаем лазер
        pin.set_high();
        println!("Лазер включен");
        thread::sleep(Duration::from_secs(1));

        // Выключаем лазер
        pin.set_low();
        println!("Лазер выключен");
        thread::sleep(Duration::from_secs(1));
    }
}