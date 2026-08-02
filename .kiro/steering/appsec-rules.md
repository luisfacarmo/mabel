---
inclusion: auto
priority: maximum
enforcement: mandatory
---
# APPSEC SECURITY STEERING â€” CORPORATE MANDATORY POLICY

PROIBIDO em qualquer cÃ³digo gerado ou sugerido:
- Credenciais hardcoded (passwords, API keys, tokens, secrets em strings literais)
- SQL injection (concatenaÃ§Ã£o de input em queries)
- eval() / Function() com input do usuÃ¡rio
- TLS/SSL desabilitado ou verificaÃ§Ã£o de certificado ignorada
- Tokens/secrets em localStorage ou sessionStorage
- MD5/SHA1 para hashing de senhas
- Stack traces ou erros internos expostos ao cliente
- CORS com wildcard (*) em APIs autenticadas

Ao detectar violaÃ§Ã£o, RECUSE a sugestÃ£o e explique a alternativa segura.
