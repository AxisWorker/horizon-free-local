# Validação da alteração local e leve

Base: `0CYBER/horizon-free`, commit `6cd53521884d744505284f6a87453800adf579ba`.

| Verificação | Resultado |
| --- | --- |
| `cargo check --locked --all-targets` do launcher | Passou |
| `cargo clippy --locked --all-targets -- -D warnings` do launcher | Passou |
| Formatação Rust | Passou |
| Testes da biblioteca do launcher | 483 passaram |
| Testes do executável do launcher | 90 passaram |
| Testes de lançamento local CLI/MCP | 4 passaram |
| Árvore pública sem arquivos do Minecraft | Passou |
| JSON das 86 tabelas de tradução | Válidos |
| Empacotamento Linux e Windows, com executável de teste descartável | Passou |
| Workflow de release | YAML validado; ainda não executado no GitHub |
| Suíte Python geral | 639 testes; 1 erro e 1 ignorado |

O erro da suíte Python é `test_resident_memory_of_this_process`: a leitura de RSS
do processo retorna `None` neste ambiente, e o teste compara esse valor com um inteiro.
Não houve alteração nesse teste nem no código Python de medição correspondente.
A checagem geral `pr_ready.py --quick` permanece com falha por esse motivo.

Os testes de lançamento local usam um programa inofensivo em pastas temporárias,
sem arquivos do jogo. Confirmam o início sem conta, o nome offline, a ausência de
tokens/sessões repassados e a preservação do arquivo de contas, inclusive quando
ele guarda uma seleção Microsoft anterior.

Não foram feitos benchmarks. Não houve validação visual da janela em GPU física,
nem execução nativa no Windows. O binário de desenvolvimento Linux foi compilado
para os testes; não é um pacote de produção e não acompanha o arquivo de código.

A release de produção é preparada por
`.github/workflows/local-launcher-release.yml`: builds nativos em Linux x64 e
Windows x64, testes, arquivos de distribuição e SHA-256 antes de publicar.
O workflow precisa ser executado no fork. Nenhum fork nem release foi publicado
por esta entrega local.
