// SPDX-License-Identifier: MPL-2.0

//! Core application state, update logic, event subscriptions, and UI layout for R Calc.

use cosmic::app::Core;
use cosmic::iced::keyboard::{self, Key, key::Code, key::Named, key::Physical};
use cosmic::iced::{Event, Length, Subscription, alignment, event};
use cosmic::theme;
use cosmic::widget::{button, column, container, row, text};
use cosmic::{Action, Application, Element, Task};

/// Application state storing the current display value, calculation history,
/// and flags for input sequencing.
pub struct AppModel {
    /// Core state managed by the COSMIC application runtime.
    core: Core,
    /// Current content shown on the calculator's primary display (visor).
    visor: String,
    /// History text displayed above the visor (e.g., "12 + 5 =").
    historico: String,
    /// Flag indicating whether the next entered digit should clear the visor to start a new calculation.
    nova_conta: bool,
}

impl AppModel {
    /// Evaluates the mathematical expression currently in the visor using `meval`.
    ///
    /// Updates the history line, handles potential evaluation errors or infinite/NaN results,
    /// rounds valid float results to 4 decimal places, and prepares the model for subsequent input.
    fn processar_calculo(&mut self) {
        if self.visor.trim().is_empty() || self.visor == "Error" || self.visor == "Erro" {
            return;
        }

        self.historico = format!("{} =", self.visor);

        // Replace '%' with '/100.0' to handle percentage calculations correctly
        let expressao_formatada = self.visor.replace("%", "/100.0");
        
        match meval::eval_str(&expressao_formatada) {
            Ok(resultado) => {
                if resultado.is_infinite() || resultado.is_nan() {
                    self.visor = String::from("Error");
                } else {
                    // Round to 4 decimal places to prevent floating point inaccuracies
                    let arredondado = (resultado * 10000.0).round() / 10000.0;
                    self.visor = arredondado.to_string();
                }
            }
            Err(_) => {
                self.visor = String::from("Error");
            }
        }
        self.nova_conta = true;
    }
}

/// Messages emitted by user interactions (UI buttons or keyboard events).
#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    /// A numeric digit, decimal separator ('.'), or operator ('+', '-', '*', '/', '%') was entered.
    DigitoPressionado(char),
    /// Clear the entire calculation and display ("C").
    Limpar,
    /// Delete the last entered character (Backspace).
    Back,
    /// Calculate and display the expression result ("=").
    Calcular,
    /// Copy the current display value to the system clipboard (Ctrl+C).
    Copiar,
}

impl Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    /// RDNN application identifier matching the desktop entry and metainfo.
    const APP_ID: &'static str = "io.github.alexkley.r_calc";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application model with default values.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        (
            AppModel {
                core,
                visor: String::from("0"),
                historico: String::new(),
                nova_conta: true,
            },
            Task::none(),
        )
    }

    /// Processes application messages and updates model state.
    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::DigitoPressionado(c) => {
                // If display shows an error or a leading "0" before entering a digit, clear display first
                if self.visor == "Error" || self.visor == "Erro" || (self.visor == "0" && c.is_ascii_digit()) {
                    self.visor.clear();
                }

                // If a calculation was just completed:
                if self.nova_conta {
                    // When user types a digit, start fresh with the new number.
                    // If an operator (+, -, *, /, %) is pressed instead, keep the previous result
                    // to allow continuing calculations seamlessly.
                    if c.is_ascii_digit() {
                        self.visor.clear();
                    }
                    self.nova_conta = false;
                }

                self.visor.push(c);
            }
            Message::Limpar => {
                self.visor = String::from("0");
                self.historico.clear();
                self.nova_conta = true;
            }
            Message::Back => {
                // If the display shows an error, Backspace resets to "0" like Clear
                if self.visor == "Error" || self.visor == "Erro" {
                    self.visor = String::from("0");
                    self.nova_conta = true;
                } else {
                    self.visor.pop();

                    // If removing the last character leaves the string empty, reset to "0"
                    if self.visor.is_empty() {
                        self.visor = String::from("0");
                        self.nova_conta = true;
                    }
                }
            }
            Message::Calcular => {
                self.processar_calculo();
            }
            Message::Copiar => {
                return cosmic::iced::clipboard::write(self.visor.clone());
            }
        }
        Task::none()
    }

    /// Subscribes to keyboard events for keyboard navigation, numpad input, and shortcuts.
    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                physical_key,
                modifiers,
                ..
            }) => {
                // Handle clipboard copy (Ctrl+C / Ctrl+c)
                if modifiers.control() {
                    if let Key::Character(c) = &key
                        && (c.as_str() == "c" || c.as_str() == "C")
                    {
                        return Some(Message::Copiar);
                    }
                    return None; // Ignore any other Ctrl shortcuts to avoid unwanted text entry
                }

                // 1. Try to match logical keys from the primary keyboard
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
                    Key::Named(Named::Escape | Named::Delete) => Some(Message::Limpar),
                    _ => None,
                };

                if mensagem_logica.is_some() {
                    return mensagem_logica;
                }

                // 2. Hardware fallback: Direct mapping of physical Numpad keys
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

    /// Renders the user interface using COSMIC widgets and styling.
    fn view(&self) -> Element<'_, Self::Message> {
        let texto_historico = text(&self.historico)
            .size(20)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right);

        let texto_display = text(&self.visor)
            .size(40)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Right);

        let coluna_visor = column![texto_historico, texto_display].spacing(5);

        let display = container(coluna_visor)
            .width(Length::Fixed(286.0))
            .height(Length::Fixed(100.0))
            .padding(10)
            .class(theme::Container::Card);

        // Standard square button constructor
        let btn = |texto: &'static str, msg: Message, estilo: theme::Button| {
            button::text(texto)
                .on_press(msg)
                .class(estilo)
                .width(Length::Fixed(64.0))
                .height(Length::Fixed(64.0))
        };

        // Wide button constructor for the "0" key (spans across 2 columns)
        let btn_largo = |texto: &'static str, msg: Message, estilo: theme::Button| {
            button::text(texto)
                .on_press(msg)
                .class(estilo)
                .width(Length::Fixed(138.0)) // 64 + 10 (spacing) + 64
                .height(Length::Fixed(64.0))
        };

        // Button colors and styles according to COSMIC HIG:
        // - Destructive (Red accent) for Clear and Backspace
        // - Suggested (Accent blue) for the equals button
        // - Standard (Neutral) for digits and operators

        let linha_1 = row![
            btn("C", Message::Limpar, theme::Button::Destructive),
            btn("⌫", Message::Back, theme::Button::Destructive),
            btn("%", Message::DigitoPressionado('%'), theme::Button::Standard),
            btn("/", Message::DigitoPressionado('/'), theme::Button::Standard),
        ]
        .spacing(10);

        let linha_2 = row![
            btn("7", Message::DigitoPressionado('7'), theme::Button::Standard),
            btn("8", Message::DigitoPressionado('8'), theme::Button::Standard),
            btn("9", Message::DigitoPressionado('9'), theme::Button::Standard),
            btn("*", Message::DigitoPressionado('*'), theme::Button::Standard),
        ]
        .spacing(10);

        let linha_3 = row![
            btn("4", Message::DigitoPressionado('4'), theme::Button::Standard),
            btn("5", Message::DigitoPressionado('5'), theme::Button::Standard),
            btn("6", Message::DigitoPressionado('6'), theme::Button::Standard),
            btn("-", Message::DigitoPressionado('-'), theme::Button::Standard),
        ]
        .spacing(10);

        let linha_4 = row![
            btn("1", Message::DigitoPressionado('1'), theme::Button::Standard),
            btn("2", Message::DigitoPressionado('2'), theme::Button::Standard),
            btn("3", Message::DigitoPressionado('3'), theme::Button::Standard),
            btn("+", Message::DigitoPressionado('+'), theme::Button::Standard),
        ]
        .spacing(10);

        let linha_acoes = row![
            btn_largo("0", Message::DigitoPressionado('0'), theme::Button::Standard),
            btn(".", Message::DigitoPressionado('.'), theme::Button::Standard),
            btn("=", Message::Calcular, theme::Button::Suggested),
        ]
        .spacing(10);

        // Group the display and button rows into a vertical column
        let grade = column![display, linha_1, linha_2, linha_3, linha_4, linha_acoes]
            .spacing(20)
            .padding(20);

        // Place the calculator grid inside an expanding container centered both horizontally and vertically
        container(grade)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn test_eval_basic_arithmetic() {
        assert_eq!(meval::eval_str("2 + 2").unwrap(), 4.0);
        assert_eq!(meval::eval_str("10 - 3").unwrap(), 7.0);
        assert_eq!(meval::eval_str("4 * 5").unwrap(), 20.0);
        assert_eq!(meval::eval_str("15 / 3").unwrap(), 5.0);
    }

    #[test]
    fn test_rounding_precision() {
        let val = (meval::eval_str("1 / 3").unwrap() * 10000.0).round() / 10000.0;
        assert_eq!(val, 0.3333);
    }

    #[test]
    fn test_eval_syntax_error() {
        assert!(meval::eval_str("2++").is_err());
        assert!(meval::eval_str("abc").is_err());
    }

    #[test]
    fn test_message_equality() {
        assert_eq!(Message::DigitoPressionado('5'), Message::DigitoPressionado('5'));
        assert_ne!(Message::Limpar, Message::Calcular);
    }
}