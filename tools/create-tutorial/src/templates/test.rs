/// Generates the e2e test template
pub fn generate_test(slug: &str) -> String {
    format!(r#"import {{ describe, it, expect }} from 'vitest';
import {{ ApiPromise, WsProvider }} from '@polkadot/api';
import net from 'node:net';

async function isPortReachable(host: string, port: number, timeoutMs: number): Promise<boolean> {{
  return new Promise((resolve) => {{
    const socket = new net.Socket();
    const done = (ok: boolean) => {{ try {{ socket.destroy(); }} catch {{}} ; resolve(ok); }};
    socket.setTimeout(timeoutMs);
    socket.once('error', () => done(false));
    socket.once('timeout', () => done(false));
    socket.connect(port, host, () => done(true));
  }});
}}

describe('{} e2e', () => {{
  it('connects and reads chain info', async () => {{
    const endpoint = process.env.POLKADOT_WS || 'ws://127.0.0.1:9944';
    const {{ hostname, port }} = new URL(endpoint.replace('ws://', 'http://'));
    if (!(await isPortReachable(hostname, Number(port || 9944), 1000))) {{
      console.log('⏭️  Skipping test - node not available');
      return;
    }}

    const api = await ApiPromise.create({{ provider: new WsProvider(endpoint, 1) }});
    const header = await api.rpc.chain.getHeader();
    expect(header.number.toNumber()).toBeGreaterThanOrEqual(0);
    await api.disconnect();
  }});
}});
"#, slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_includes_slug() {
        let test = generate_test("my-tutorial");
        assert!(test.contains("my-tutorial e2e"));
    }

    #[test]
    fn test_generate_test_has_vitest_imports() {
        let test = generate_test("test");
        assert!(test.contains("import { describe, it, expect } from 'vitest'"));
    }

    #[test]
    fn test_generate_test_has_polkadot_api() {
        let test = generate_test("test");
        assert!(test.contains("@polkadot/api"));
        assert!(test.contains("ApiPromise"));
    }
}
