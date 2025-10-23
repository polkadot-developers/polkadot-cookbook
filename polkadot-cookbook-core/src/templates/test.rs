/// Test file template generator
use super::Template;

/// Generates e2e test content for a tutorial project
pub struct TestTemplate {
    slug: String,
}

impl TestTemplate {
    pub fn new(slug: impl Into<String>) -> Self {
        Self { slug: slug.into() }
    }
}

impl Template for TestTemplate {
    fn generate(&self) -> String {
        format!(
            r#"import {{ describe, it, expect }} from 'vitest';
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
"#,
            self.slug
        )
    }
}

/// Legacy function for backward compatibility
pub fn generate_test(slug: &str) -> String {
    TestTemplate::new(slug).generate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_includes_slug() {
        let template = TestTemplate::new("my-tutorial");
        let test = template.generate();
        assert!(test.contains("my-tutorial e2e"));
    }

    #[test]
    fn test_generate_test_has_vitest_imports() {
        let template = TestTemplate::new("test");
        let test = template.generate();
        assert!(test.contains("import { describe, it, expect } from 'vitest'"));
    }

    #[test]
    fn test_generate_test_has_polkadot_api() {
        let template = TestTemplate::new("test");
        let test = template.generate();
        assert!(test.contains("@polkadot/api"));
        assert!(test.contains("ApiPromise"));
    }

    #[test]
    fn test_legacy_function() {
        let test = generate_test("my-tutorial");
        assert!(test.contains("my-tutorial e2e"));
    }
}
