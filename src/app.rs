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
    nova_conta: bool,
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
                visor: String::new(),
                nova_conta: false,
            },
            Task::none(),
        )
    }

    // Retorno atualizado para encapsular no Action
    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::DigitoPressionado(c) => {
                // Se a tela exibe um erro, limpamos imediatamente ao digitar qualquer coisa
                if self.visor == "Erro" || (self.visor == "0" && c.is_ascii_digit()) {
                    self.visor.clear();
                }
                // Se acabamos de fazer uma conta...
                if self.nova_conta {
                    // E o usuário digitou um NÚMERO, limpamos a tela para começar de novo.
                    // (Se ele digitou um '+', '-', '*' ou '/', a tela NÃO limpa, 
                    // permitindo que ele continue a conta com o resultado anterior!)
                    if c.is_ascii_digit() {
                        self.visor.clear();
                    }
                    // A conta não é mais nova, voltamos ao modo de digitação normal
                    self.nova_conta = false;
                }

                self.visor.push(c);
            }
            Message::Limpar => {
                self.visor = String::from("0");
                self.nova_conta = true;
            }
            Message::Back => {
                // Se a tela estiver mostrando "Erro", o backspace funciona como o "C" (Limpar)
                if self.visor == "Erro" {
                    self.visor = String::from("0");
                    self.nova_conta = true;
                } else {
                    // Remove o último caractere da string
                    self.visor.pop();
                    
                    // Se a remoção deixou a string completamente vazia, colocamos o "0"
                    if self.visor.is_empty() {
                        self.visor = String::from("0");
                        self.nova_conta = true; // Garante que o "0" suma ao digitar um novo número
                    }
                }
            }
            Message::Calcular => {
                match meval::eval_str(&self.visor) {
                    Ok(resultado) => {
                        // Bloqueia divisões por zero ou resultados impossíveis (NaN)
                        if resultado.is_infinite() || resultado.is_nan() {
                            self.visor = String::from("Error");
                        } else {
                            // Multiplica por 10.000, arredonda e divide para manter 4 casas decimais.
                            // O .to_string() lidará sozinho com a remoção de zeros desnecessários no final.
                            let arredondado = (resultado * 10000.0).round() / 10000.0;
                            self.visor = arredondado.to_string();
                        }
                        self.nova_conta = true;
                    }
                    Err(_) => {
                        self.visor = String::from("Error");
                        self.nova_conta = true;
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
                    Key::Named(Named::Escape) | Key::Named(Named::Delete) => Some(Message::Limpar),
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

    fn view(&self) -> Element<'_, Self::Message> {
        let texto_display = text(&self.visor)
            .size(40)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            .align_y(alignment::Vertical::Center);

        let display = container(texto_display)
            .width(Length::Fixed(286.0))
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

        // 1. Guardamos a nossa grade inteira em uma variável
        let grade = column![display, linha_1, linha_2, linha_3, linha_4, linha_acoes]
            .spacing(20)
            .padding(20);

        // 2. Colocamos a grade dentro de um Container invisível que se expande e centraliza tudo
        container(grade)
            .width(Length::Fill)   // Estica para preencher a largura do "tile"
            .height(Length::Fill)  // Estica para preencher a altura do "tile"
            .align_x(alignment::Horizontal::Center) // Centraliza na horizontal
            .align_y(alignment::Vertical::Center)   // Centraliza na vertical
            .into()
        }
}