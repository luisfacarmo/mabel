# INICIAR AQUI — Investigacao de Protocolo Soundcore A3062

> **Status:** Conexao RFCOMM estabelecida mas comandos nao surtem efeito no device.
> **Ultima sessao:** 02/08/2026
> **Prioridade:** Resolver antes de qualquer outra task.

---

## Problema actual

O Mabel conecta ao Space One Pro via RFCOMM service [1] (UUID `00001108` — Headset HS).
A conexao e aceite e packets sao enviados (write funciona sem erros).
Porem:
1. O device **nao responde** com state updates (read channel vazio)
2. Os comandos enviados **nao causam efeito audivel** (ANC nao muda, EQ nao muda)
3. O frontend mostra "Offline" porque nunca recebe `device-state` event

**Causa provavel:** Estamos conectados ao servico errado. O UUID `00001108` e o Headset profile (controlo de audio), nao o canal de dados proprietario do Soundcore onde o protocolo `0x08/0x09` opera.

---

## O que ja sabemos

### Servicos RFCOMM disponiveis no Space One Pro (6 total)

| Index | UUID | Perfil BT | Status |
|-------|------|-----------|--------|
| 0 | 0000111E-0000-1000-8000-00805F9B34FB | Handsfree (HFP) | Ocupado pelo Windows |
| 1 | 00001108-0000-1000-8000-00805F9B34FB | Headset (HSP) | Conecta mas sem dados |
| 2-5 | **Desconhecidos** | Nao foram listados | **Precisam ser investigados** |

### O que o OpenSCQ30 faz

O OpenSCQ30 usa `RfcommServiceSelectionStrategy::Dynamic` que:
1. Lista todos os servicos
2. Deixa um callback escolher o UUID correto
3. Para o A3062 especificamente, pode usar um UUID custom ou SPP (`00001101`)

**Ficheiro de referencia:** `OpenSCQ30/lib/src/connection_backend/windows/rfcomm.rs`

---

## Investigacao necessaria

### 1. Listar TODOS os 6 UUIDs

O bug actual no log e que o loop para no primeiro servico que conecta.
Precisamos logar todos os 6 UUIDs antes de tentar conectar.
Provavelmente um deles e `00001101` (SPP) que e o canal de dados real.

### 2. Capturar HCI Snoop Log

Usar o Android Developer Options para capturar o trafego BT quando o app oficial Soundcore se conecta:
1. Settings > Developer Options > Enable Bluetooth HCI Snoop Log
2. Abrir Soundcore app no telemovel
3. Conectar ao Space One Pro
4. Mudar ANC mode (para ver o comando no log)
5. Extrair o btsnoop_hci.log
6. Abrir no Wireshark com filtro `btrfcomm`
7. Identificar qual channel/UUID o app oficial usa

### 3. Analisar o APK do Soundcore

Ferramentas:
- `jadx` para decompile do APK
- Procurar por UUIDs de servico hardcoded
- Procurar por `00001101` ou UUIDs custom
- Keyword: `RfcommSocket`, `BluetoothSocket`, `createRfcommSocket`

### 4. Verificar OpenSCQ30 Issues/PRs

O OpenSCQ30 ja fez este trabalho para o A3062. Verificar:
- https://github.com/Oppzippy/OpenSCQ30/issues?q=A3062
- Como o `RfcommServiceSelectionStrategy` e configurado para este modelo
- Qual UUID especifico e selecionado

---

## Links uteis para investigacao

### Comunidade / Discussoes
- [Space One Pro Firmware 4.33 (Reddit)](https://reddit.com/r/soundcore)
- [Temas 4PDA Space One Pro](https://4pda.to/forum/index.php?showtopic=1094161)

### Engenharia Reversa BLE/RFCOMM
- [Reverse Engineering BLE Devices (ReadTheDocs)](https://reverse-engineering-ble-devices.readthedocs.io/)
- [nRF Connect for Mobile](https://www.nordicsemi.com/Products/Development-tools/nRF-Connect-for-mobile)
- [Bleak (Python BLE)](https://github.com/hbldh/bleak)
- [BlueZ (Linux BT Stack)](http://www.bluez.org/)
- [Wireshark BT dissectors](https://www.wireshark.org/)

### Codigo-fonte de referencia
- [OpenSCQ30 (principal)](https://github.com/Oppzippy/OpenSCQ30)
- [SoundcoreManager](https://github.com/nicoboss/SoundcoreManager)

### Pesquisas recomendadas no GitHub
```
github soundcore reverse engineering
github soundcore ble
soundcore protocol
soundcore gatt
soundcore bluetooth reverse engineering
anker soundcore apk reverse
soundcore apk jadx
bluetooth hci snoop soundcore
```

### Documentacao oficial
- [Soundcore Space One Pro (pagina oficial)](https://www.soundcore.com/products/space-one-pro)

---

## Hipoteses de solucao (ordenadas por probabilidade)

### Hipotese A: SPP Channel errado (mais provavel)
O Soundcore protocol usa SPP (`00001101`) mas num channel number especifico (nao o default).
O OpenSCQ30 seleciona o servico dinamicamente baseado nos UUIDs disponiveis.
**Accao:** Listar todos os 6 UUIDs e tentar o SPP (`00001101`) especificamente.

### Hipotese B: Precisa de handshake antes dos comandos
O device pode precisar de receber um pacote de "inicio de sessao" antes de aceitar comandos.
O `request_state()` packet `[0x01, 0x01]` e esse handshake — mas talvez so funcione no canal correcto.
**Accao:** Verificar se ha resposta ao request_state quando conectado ao SPP.

### Hipotese C: Ordem dos bytes no packet esta errada
Improvavel — o parser passa nos 37 testes com o test vector real do OpenSCQ30.
Mas o `to_bytes()` outbound pode ter um detalhe errado (direction header, checksum).
**Accao:** Capturar um packet real do Soundcore app e comparar byte a byte.

### Hipotese D: Firmware version incompativel
O test vector e de firmware `03.37`. Se o fone tem firmware mais recente (`04.33`+),
o formato dos packets pode ter mudado.
**Accao:** Verificar firmware actual do fone e comparar com o que o OpenSCQ30 suporta.

---

## Quick fix para testar imediatamente

Antes de investigacao profunda, testar isto:
1. Modificar `connect_blocking()` para logar TODOS os 6 UUIDs sem tentar conectar
2. Identificar se `00001101` (SPP) esta na lista
3. Se estiver, forcar conexao nesse UUID especificamente
4. Se nao estiver, verificar quais UUIDs custom existem

---

## Contexto tecnico

### Wire format do protocolo Soundcore (confirmado)
```
[direction: 5 bytes] [command: 2 bytes] [length: 2 bytes LE] [body: N bytes] [checksum: 1 byte]
```
- Outbound: `[0x08, 0xEE, 0x00, 0x00, 0x00]`
- Inbound: `[0x09, 0xFF, 0x00, 0x00, 0x01]`
- Checksum: sum of all preceding bytes mod 256

### State request packet
```
08 EE 00 00 00 01 01 0A 00 02
```
(direction=outbound, command=[0x01,0x01], length=10, body=empty, checksum=0x02)

### Testes que passam: 37
- Packet framing roundtrip
- A3062 state parser (battery, firmware, serial, EQ, ANC, toggles)
- Command builders (9 commands)
- Streaming parser (fragmentation, garbage skip)
