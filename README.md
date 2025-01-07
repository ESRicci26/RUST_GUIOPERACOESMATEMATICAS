Calculadora de Operações Básicas em GTK e Relm

Este projeto implementa uma calculadora simples utilizando as bibliotecas GTK e Relm no Rust.
A calculadora realiza operações de adição, subtração, multiplicação, divisão e limpeza de campos. A interface gráfica foi desenvolvida usando o GTK,
com o modelo de programação funcional fornecido pelo Relm.

Funcionalidades
Multiplicação: Multiplica os dois números inseridos nos campos de entrada.
Adição: Adiciona os dois números inseridos nos campos de entrada.
Subtração: Subtrai o segundo número do primeiro número inserido nos campos de entrada.
Divisão: Divide o primeiro número pelo segundo. Se o divisor for zero, exibe uma mensagem de erro.
Limpeza de Campos: Limpa todos os campos de entrada e o campo de resultado.
Estrutura do Projeto
O projeto utiliza as bibliotecas GTK e Relm para criar a interface gráfica e gerenciar eventos. O modelo de mensagens do Relm é utilizado para definir os eventos da interface, como multiplicação, adição, subtração, divisão e limpeza dos campos.

Dependências
No arquivo Cargo.toml do projeto, as dependências são as seguintes:

[package]
name = "GUIOPERACOESMATEMATICAS"
version = "0.1.0"
edition = "2021"

[dependencies]
gtk = "0.16.1"
relm = "0.24.0"
relm-derive = "0.24.0"


gtk: Biblioteca para criar interfaces gráficas.
relm: Biblioteca que facilita a criação de aplicativos com programação reativa e baseada em mensagens.
relm-derive: Macros necessárias para o uso do Relm.
Código Principal (src/main.rs)
O código da aplicação consiste em uma janela que contém dois campos de entrada para os números, botões para cada operação (multiplicar, somar,
subtrair, dividir e limpar) e um campo para exibir o resultado da operação.

Estrutura do código:
Model: Define o modelo de dados do aplicativo.
Msg: Define as mensagens (eventos) que a interface gráfica pode gerar.
Win: Implementa a interface gráfica e a lógica de atualização dos dados, incluindo as operações matemáticas e a interação com a interface.
Funções Principais
1. Msg::MultiplyNumbers
Realiza a multiplicação dos números inseridos nos campos de entrada.

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
2. Msg::AddNumbers
Realiza a soma dos números inseridos nos campos de entrada.

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
3. Msg::SubtractNumbers
Realiza a subtração do segundo número do primeiro número inserido.

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
4. Msg::DivideNumbers
Realiza a divisão do primeiro número pelo segundo número. Se o divisor for zero, exibe uma mensagem de erro.

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
5. Msg::ClearFields
Limpa todos os campos de entrada e o campo de resultado.

Msg::ClearFields => {
    // Limpa todos os campos de entrada
    self.widgets.text1.set_text("");
    self.widgets.text2.set_text("");
    self.widgets.text3.set_text("");
}
Interface Gráfica
A interface gráfica consiste em uma janela principal que contém:

Dois campos de entrada (Entry): para inserir os números.
Botões para as operações:
Multiplicar: Executa a multiplicação dos números.
Adicionar: Executa a adição dos números.
Subtrair: Executa a subtração dos números.
Dividir: Executa a divisão dos números (verifica divisão por zero).
Limpar: Limpa todos os campos.
Campo de texto (Entry): para mostrar o resultado da operação.
Como Executar o Projeto
Clone o repositório para o seu computador:

Certifique-se de ter o Rust instalado. Caso não tenha, instale-o a partir de https://www.rust-lang.org.

Compile e execute o projeto:
cargo run
A janela da calculadora será aberta e você pode usar as funcionalidades de multiplicar, adicionar, subtrair, dividir e limpar.

Essa documentação oferece uma visão completa do projeto, como utilizá-lo, as dependências, e explica cada parte do código.
