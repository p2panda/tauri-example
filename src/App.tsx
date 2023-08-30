import React, { useEffect, useMemo, useState } from 'react';
import { Session, KeyPair, initWebAssembly } from 'shirokuma';

import "./App.css";

type Props = {
  children: React.ReactNode;
};

export const InitWasm = ({ children }: Props) => {
  const [ready, setReady] = useState(false);

  useEffect(() => {
    const init = async () => {
      await initWebAssembly();
      setReady(true);
    };

    init();
  }, []);

  return ready ? children : null;
};

function App() {
  const { keyPair, session } = useMemo(() => {
    const keyPair = new KeyPair();
    const session = new Session('http://localhost:2020/graphql')
      .setKeyPair(keyPair);
    return { keyPair, session };
  }, []);

  return (
    <InitWasm>
      <div className="container">
        <h1>Hello, Panda!</h1>
        <p>Public key: {keyPair.publicKey()}</p>
      </div>
    </InitWasm>
  );
}

export default App;
