import './styles/CreateBoardButton.css'

function CreateBoardButton({ onCreate, variant = 'button' }) {
    function handleCreate() {
        if (onCreate)
            onCreate()
    }

    return (
        <div className={`container_create-board--${variant}`}>
            <div className={`create-board-button create-board-button--${variant}`} onClick={ handleCreate }>
                <h3 className='create-board-button_label'>
                    <span className='create-board-button_plus' aria-hidden='true'></span>
                    Create new Board
                </h3>
            </div>
        </div>
    );
}

export default CreateBoardButton;
