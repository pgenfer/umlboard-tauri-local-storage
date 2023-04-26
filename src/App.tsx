import reactLogo from "./assets/react.svg";
import "./App.css";
import { useDispatch } from "react-redux";
import { useRootSelector } from "./store";
import { invoke } from "@tauri-apps/api";
import { cancelClassifierRename, renameClassifier, renamingClassifier } from "./edit-name.redux";
import { useEffect } from "react";
import { applicationReady } from "./app.redux";
import { selectFirstClassifier } from "./classifier-selector";

type IpcMessage = {
  domain: string,
  action: {type: string, payload: any}
}

function App() {
  
  const name = useRootSelector(state => state.classifier.currentName);
  const editState = useRootSelector(state => state.classifier.editState);
  const currentClassifier = useRootSelector(selectFirstClassifier);
  const dispatch = useDispatch();

  async function sendMessage<T>(action: {type: string, payload: T}) {
    const actionIdentifiers = action.type.split("/");
    const domain = actionIdentifiers[0]
    const type = actionIdentifiers[1];
    const message: IpcMessage = {
      domain,
      action: {...action, type }
    };
    const answer = await invoke<IpcMessage>("ipc_message", {message} );
    const responseAction = {
      ...answer.action,
      type: `${answer.domain}/${answer.action.type}`
    };
    dispatch(responseAction);
  }

  useEffect(() => {
    console.log('application initialized');
    const sendApplicationReady = async () => {
      await sendMessage(applicationReady());
    }
    sendApplicationReady();
  }, []);

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>This is a small concept app to demonstrate state persistence with Tauri/Rust.<br/>
      Change the name and press a button to confirm or cancel the change.</p>
      <br/>
      <div className="row">
        {currentClassifier && <div>
          <input
            id="greet-input"
            value={name}
            onChange={(e) => dispatch(renamingClassifier({newName: e.target.value, id: currentClassifier.id}))}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={async () => {
            await sendMessage(renameClassifier({newName: name, id: currentClassifier.id}));
          }}>
            Edit
          </button>
          <button type="button" onClick={async () => {
            await sendMessage(cancelClassifierRename());
          }}>
            Cancel
          </button>
        </div>}
      </div>
      {editState !== 'none' && <p className={editState === 'successful' ? 'change-successful' : 'change-canceled'}>
        {editState === 'successful' ? 'Name was changed' : 'Editing was canceled.'}
      </p>}
    </div>
  );
}

export default App;
