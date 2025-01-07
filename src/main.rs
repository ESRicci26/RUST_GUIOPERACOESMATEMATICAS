use gtk::prelude::*;
use relm::Widget;
use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    MultiplyNumbers,
    AddNumbers,
    SubtractNumbers,
    DivideNumbers,
    ClearFields,
    Quit,
}

pub struct Model {}

#[relm_derive::widget]
impl Widget for Win {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::MultiplyNumbers => {
                // Obtém texto dos campos de entrada
                let text1 = self.widgets.text1.text().as_str().to_string();
                let text2 = self.widgets.text2.text().as_str().to_string();

                // Converte os textos em números
                let num1: f64 = text1.parse().unwrap_or(0.0);
                let num2: f64 = text2.parse().unwrap_or(0.0);

                // Multiplica os números
                let result = num1 * num2;

                // Exibe o resultado no campo de texto
                self.widgets.text3.set_text(&result.to_string());
            }
            Msg::AddNumbers => {
                // Obtém texto dos campos de entrada
                let text1 = self.widgets.text1.text().as_str().to_string();
                let text2 = self.widgets.text2.text().as_str().to_string();

                // Converte os textos em números
                let num1: f64 = text1.parse().unwrap_or(0.0);
                let num2: f64 = text2.parse().unwrap_or(0.0);

                // Soma os números
                let result = num1 + num2;

                // Exibe o resultado no campo de texto
                self.widgets.text3.set_text(&result.to_string());
            }
            Msg::SubtractNumbers => {
                // Obtém texto dos campos de entrada
                let text1 = self.widgets.text1.text().as_str().to_string();
                let text2 = self.widgets.text2.text().as_str().to_string();

                // Converte os textos em números
                let num1: f64 = text1.parse().unwrap_or(0.0);
                let num2: f64 = text2.parse().unwrap_or(0.0);

                // Subtrai os números
                let result = num1 - num2;

                // Exibe o resultado no campo de texto
                self.widgets.text3.set_text(&result.to_string());
            }
            Msg::DivideNumbers => {
                // Obtém texto dos campos de entrada
                let text1 = self.widgets.text1.text().as_str().to_string();
                let text2 = self.widgets.text2.text().as_str().to_string();

                // Converte os textos em números
                let num1: f64 = text1.parse().unwrap_or(0.0);
                let num2: f64 = text2.parse().unwrap_or(0.0);

                // Verifica se o divisor não é zero
                if num2 != 0.0 {
                    // Divide os números
                    let result = num1 / num2;

                    // Exibe o resultado no campo de texto
                    self.widgets.text3.set_text(&result.to_string());
                } else {
                    self.widgets.text3.set_text("Erro: Divisão por zero");
                }
            }
            Msg::ClearFields => {
                // Limpa todos os campos de entrada
                self.widgets.text1.set_text("");
                self.widgets.text2.set_text("");
                self.widgets.text3.set_text("");
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Calculadora de Operações",
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                spacing: 5,

                // Label e campo para Número 1
                gtk::Label { label: "Número 1" },
                #[name="text1"] gtk::Entry {},

                // Label e campo para Número 2
                gtk::Label { label: "Número 2" },
                #[name="text2"] gtk::Entry {},

                // Botões de operação
                gtk::Button {
                    label: "Multiplicar",
                    clicked => Msg::MultiplyNumbers,
                },
                gtk::Button {
                    label: "Adicionar",
                    clicked => Msg::AddNumbers,
                },
                gtk::Button {
                    label: "Subtrair",
                    clicked => Msg::SubtractNumbers,
                },
                gtk::Button {
                    label: "Dividir",
                    clicked => Msg::DivideNumbers,
                },

                // Botão para limpar
                gtk::Button {
                    label: "Limpar",
                    clicked => Msg::ClearFields,
                },

                // Label e campo para Resultado
                gtk::Label { label: "Resultado" },
                #[name="text3"] gtk::Entry { editable: false }, // Campo não editável
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Erro ao iniciar o programa.");
}


/* 
use gtk::prelude::*;
use relm::Widget;
use relm_derive::Msg;

#[derive(Msg)]
pub enum Msg {
    MultiplyNumbers,
    Quit,
}

pub struct Model {}

#[relm_derive::widget]
impl Widget for Win {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::MultiplyNumbers => {
                // Obtém texto dos campos de entrada
                let text1 = self.widgets.text1.text().as_str().to_string();
                let text2 = self.widgets.text2.text().as_str().to_string();

                // Converte os textos em números
                let num1: f64 = text1.parse().unwrap_or(0.0);
                let num2: f64 = text2.parse().unwrap_or(0.0);

                // Multiplica os números
                let result = num1 * num2;

                // Exibe o resultado no campo de texto
                self.widgets.text3.set_text(&result.to_string());
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            title: "Operações Matemáticas",
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                spacing: 5,

                // Label e campo para Número 1
                gtk::Label { label: "Número 1" },
                #[name="text1"] gtk::Entry {},

                // Label e campo para Número 2
                gtk::Label { label: "Número 2" },
                #[name="text2"] gtk::Entry {},

                // Botão para multiplicar
                gtk::Button {
                    label: "Multiplicar",
                    clicked => Msg::MultiplyNumbers,
                },

                // Label e campo para Resultado
                gtk::Label { label: "Resultado" },
                #[name="text3"] gtk::Entry { editable: false }, // Campo não editável
            },
            delete_event(_, _) => (Msg::Quit, gtk::Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Erro ao iniciar o programa.");
}
*/    