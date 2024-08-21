# Projeto-Nivelamento-Robotica-Movel
Esse projeto visa analisar um pouco do seu entendimento sobre Robotica Movel, num curto espaço de tempo, utilizando a linguagem Rust e aplicando suas noções de lógica de programação, estrutura de dados, orientação a objetos e robotica movel.

## Enunciado
### JOGO DA ENTREGA
O jogo consiste em fazer com que o carteiro (**&**) leve a caixa (**@**) até o ponto desejado (**X**) em um campo que será uma matriz 20x20, onde (**+**) representa um espaço válido. 
Alem disso, o carteiro possui um sistema de "desequilíbrio", portanto é sua função equilibrá-lo. Para isso, vocês devem implementar algum algoritmo de controle. No projeto já existe a classe PID. Você pode usa-lá ou criar outra.
Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas `carteiro`, `caixa`, `jogo`, e `PID` (como exemplo de estrutura para o algoritmo de controle).
Regras:  
- O carteiro só pode andar um *índice / casa* por iteração
- Para que o carteiro ande para uma certa direção ele precisa estar virado para ela
- Apliquem a ideia de Encapsulamento
- Utilize a tecnologia ou algoritmo que achar mais conveniente

## Facilitando sua vida

### Referências
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [Resumão de Rust em 10 Minutos](https://www.youtube.com/watch?v=br3GIIQeefY)
- [Resumo Sobre Controle PID](https://www.youtube.com/watch?v=LMhjjwnojU0&t=16s) 

*Infelizmente todos os links são em inglês, Rust ainda não é tão popular no Brasil, então há pouco conteúdo em PT-BR*

### Git
É ideal que cada time utilize um repositório no GitHub para desenvolver o projeto. Assim podemos acompanhar o progresso de cada time e ajudar no que for necessário.   
Dê um [fork](https://medium.com/@abnerborgonha/como-fazer-um-fork-de-uma-projeto-no-github-9c78f2899bac) no [repositório do projeto](https://github.com/MarceloRother/Projeto-Nivelamento-Robotica-Movel) e desenvolva o projeto nele.

Uma boa prática é separar uma branch só para desenolvimento, normalmente chamada de `dev` ou `development`. E quando uma nova funcionalidade estiver pronta, você mescla essa branch com a `main`. Evitando problemas de compatibilidade  e mantendo a `main` sempre funcional. É assim que trabalhamos no SPL. **#MelhorCategoria**

### Ambiente de Desenvolvimento
Para desenvolver em Rust, é preciso baixar e instalar o compilador da linguagem. Você pode fazer isso através do [site oficial](https://www.rust-lang.org/tools/install).  
Além disso, recomendo utilizar a IDE [Visual Studio Code](https://code.visualstudio.com/) com a extensão [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) que permite rodar o código dentro do editor, autocomplete e outras funcionalidades.  
Sempre preste atenção nas mensagens do compilador, muitas vezes ele te dá dicas de como resolver um problema. Também recomendo dar uma lida nas utilidades do [Cargo](https://doc.rust-lang.org/cargo/), o gerenciador de pacotes e compilador da linguagem.