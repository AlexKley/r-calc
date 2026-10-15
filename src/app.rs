use cosmic::app::Core;
use cosmic::{Action, Application, Element, Task};
use cosmic::widget::{button, column, row, text, container};
use cosmic::theme;

use cosmic::iced::{Subscription, Length, alignment};
use cosmic::iced::{event, Event};
use cosmic::iced::keyboard::{self, Key, key::Named, key::Physical, key::Code};

// 1. O Estado da Calculadora
pub struct AppModel {
    core: Core,
    visor: String,
}

// 2. Os Eventos Possíveis
#[derive(Clone, Debug)]
pub enum Message {
    DigitoPressionado(char),
    Limpar,
    Back,
    Calcular,
}

// 3. A Implementação
impl Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.alexkley.Calculadora";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    // Retorno atualizado para encapsular no Action
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        (
            AppModel {
                core,
                visor: String::from("0"),
            },
            Task::none(),
        )
    }

    // Retorno atualizado para encapsular no Action
    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::DigitoPressionado(c) => {
                        // Agora limpa o visor se houver um 0 inicial OU uma mensagem de erro
                        if self.visor == "0" || self.visor == "Erro" {
                            self.visor.clear();
                        }
                        self.visor.push(c);
            }
            Message::Limpar => {
                self.visor = String::from("0");
            }
            Message::Back => {
                self.visor.pop();
                if self.visor.is_empty() {
                    self.visor = String::from("0");
                }
            }
            Message::Calcular => {
                match meval::eval_str(&self.visor) {
                    Ok(resultado) => {
                        // Se o cálculo der certo, atualiza o visor com o resultado
                        self.visor = resultado.to_string();
                    }
                    Err(_) => {
                        // Se a expressão matemática for inválida
                        self.visor = String::from("Erro");
                    }
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, _status, _window| match event {
            // Agora extraímos também a 'physical_key' (a tecla física apertada no hardware)
            Event::Keyboard(keyboard::Event::KeyPressed { key, physical_key, .. }) => {
                
                // 1. Tenta identificar pelo caractere lógico (teclado principal)
                let mensagem_logica = match key {
                    Key::Character(c) => match c.as_str() {
                        "0" => Some(Message::DigitoPressionado('0')),
                        "1" => Some(Message::DigitoPressionado('1')),
                        "2" => Some(Message::DigitoPressionado('2')),
                        "3" => Some(Message::DigitoPressionado('3')),
                        "4" => Some(Message::DigitoPressionado('4')),
                        "5" => Some(Message::DigitoPressionado('5')),
                        "6" => Some(Message::DigitoPressionado('6')),
                        "7" => Some(Message::DigitoPressionado('7')),
                        "8" => Some(Message::DigitoPressionado('8')),
                        "9" => Some(Message::DigitoPressionado('9')),
                        "+" => Some(Message::DigitoPressionado('+')),
                        "-" => Some(Message::DigitoPressionado('-')),
                        "*" => Some(Message::DigitoPressionado('*')),
                        "/" => Some(Message::DigitoPressionado('/')),
                        "." | "," => Some(Message::DigitoPressionado('.')), 
                        "%" => Some(Message::DigitoPressionado('%')),
                        "=" => Some(Message::Calcular),
                        "c" | "C" => Some(Message::Limpar),
                        _ => None,
                    },
                    Key::Named(Named::Enter) => Some(Message::Calcular),
                    Key::Named(Named::Backspace) => Some(Message::Back),
                    Key::Named(Named::Escape) => Some(Message::Limpar),
                    _ => None,
                };

                // Se achou nas teclas principais, envia a mensagem e para aqui
                if mensagem_logica.is_some() {
                    return mensagem_logica;
                }

                // 2. Fallback garantido: Mapeamento direto das teclas físicas do Numpad
                match physical_key {
                    Physical::Code(Code::Numpad0) => Some(Message::DigitoPressionado('0')),
                    Physical::Code(Code::Numpad1) => Some(Message::DigitoPressionado('1')),
                    Physical::Code(Code::Numpad2) => Some(Message::DigitoPressionado('2')),
                    Physical::Code(Code::Numpad3) => Some(Message::DigitoPressionado('3')),
                    Physical::Code(Code::Numpad4) => Some(Message::DigitoPressionado('4')),
                    Physical::Code(Code::Numpad5) => Some(Message::DigitoPressionado('5')),
                    Physical::Code(Code::Numpad6) => Some(Message::DigitoPressionado('6')),
                    Physical::Code(Code::Numpad7) => Some(Message::DigitoPressionado('7')),
                    Physical::Code(Code::Numpad8) => Some(Message::DigitoPressionado('8')),
                    Physical::Code(Code::Numpad9) => Some(Message::DigitoPressionado('9')),
                    Physical::Code(Code::NumpadAdd) => Some(Message::DigitoPressionado('+')),
                    Physical::Code(Code::NumpadSubtract) => Some(Message::DigitoPressionado('-')),
                    Physical::Code(Code::NumpadMultiply) => Some(Message::DigitoPressionado('*')),
                    Physical::Code(Code::NumpadDivide) => Some(Message::DigitoPressionado('/')),
                    Physical::Code(Code::NumpadDecimal) => Some(Message::DigitoPressionado('.')),
                    Physical::Code(Code::NumpadEnter) => Some(Message::Calcular),
                    _ => None,
                }
            }
            _ => None,
        })
    }

    fn view(&self) -> Element<Self::Message> {
        let texto_display = text(&self.visor)
            .size(40)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .align_y(alignment::Vertical::Center);

        let display = container(texto_display)
            .width(Length::Fill)
            .height(Length::Fixed(80.0))
            .padding(10)
            .class(theme::Container::Card);

        // 1. Atualizamos o ajudante para receber o tema
        let btn = |texto: &'static str, msg: Message, estilo: theme::Button| {
            button::text(texto)
                .on_press(msg)
                .class(estilo)
                .width(Length::Fixed(64.0))
                .height(Length::Fixed(64.0))
        };

        let btn_largo = |texto: &'static str, msg: Message, estilo: theme::Button| {
            button::text(texto)
                .on_press(msg)
                .class(estilo)
                .width(Length::Fixed(138.0)) // 64 + 10 + 64
                .height(Length::Fixed(64.0))
        };

        // 2. Aplicamos: 
        // Destructive (Vermelho) para apagar
        // Suggested (Azul/Destaque do Pop!_OS) para o igual
        // Standard (Fundo sólido normal) para números e operações

        let linha_1 = row![
            btn("C", Message::Limpar, theme::Button::Destructive),
            btn("⌫", Message::Back, theme::Button::Destructive),
            btn("%", Message::DigitoPressionado('%'), theme::Button::Standard),
            btn("/", Message::DigitoPressionado('/'), theme::Button::Standard),
        ].spacing(10);
            
        let linha_2 = row![
            btn("7", Message::DigitoPressionado('7'), theme::Button::Standard),
            btn("8", Message::DigitoPressionado('8'), theme::Button::Standard),
            btn("9", Message::DigitoPressionado('9'), theme::Button::Standard),
            btn("*", Message::DigitoPressionado('*'), theme::Button::Standard),
        ].spacing(10);

        let linha_3 = row![
            btn("4", Message::DigitoPressionado('4'), theme::Button::Standard),
            btn("5", Message::DigitoPressionado('5'), theme::Button::Standard),
            btn("6", Message::DigitoPressionado('6'), theme::Button::Standard),
            btn("-", Message::DigitoPressionado('-'), theme::Button::Standard),
        ].spacing(10);

        let linha_4 = row![
            btn("1", Message::DigitoPressionado('1'), theme::Button::Standard),
            btn("2", Message::DigitoPressionado('2'), theme::Button::Standard),
            btn("3", Message::DigitoPressionado('3'), theme::Button::Standard),
            btn("+", Message::DigitoPressionado('+'), theme::Button::Standard),
        ].spacing(10);

        let linha_acoes = row![
            btn("0", Message::DigitoPressionado('0'), theme::Button::Standard),
            btn(".", Message::DigitoPressionado('.'), theme::Button::Standard),
            btn_largo("=", Message::Calcular, theme::Button::Suggested),
        ].spacing(10);

        column![display, linha_1, linha_2, linha_3, linha_4, linha_acoes]
            .spacing(20)
            .padding(20)
            .into()
    }
}