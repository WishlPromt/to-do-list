import logo from './logo.svg';
import './App.css';
import BoardsContainer from './boards/BoardsContainer';

function App() {
  return (
    <div className='App'>
      <header className='App-header'>
        <div className='App-logo_container'>
          <img src={logo} className='logo' alt='logo' />
        </div>
        <h1 className='App-name'>ToDoList</h1>
      </header>
      <BoardsContainer />
    </div>
  );
}

export default App;
