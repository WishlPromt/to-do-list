import { useState } from 'react';
import './styles/CreateBoardWindow.css';

function CreateBoardWindow({ onAccept, onCancel, initialName = 'Board' }) {
    const [boardName, setBoardName] = useState(initialName);

    const trimmedName = boardName.trim();

    function handleSubmit(event) {
        event.preventDefault();

        if (!trimmedName)
            return;

        if (onAccept)
            onAccept(trimmedName);
    }

    function handleKeyDown(event) {
        if (event.key === 'Escape' && onCancel)
            onCancel();
    }

    function handleCancel() {
        if (onCancel)
            onCancel();
    }

    return (
        <div className='create-board-window' onClick={ handleCancel } onKeyDown={ handleKeyDown }>
            <form
                className='create-board-window_form'
                role='dialog'
                aria-modal='true'
                aria-labelledby='create-board-window_label'
                onClick={ (event) => event.stopPropagation() }
                onSubmit={ handleSubmit }
            >
                <label
                    id='create-board-window_label'
                    className='create-board-window_label'
                    htmlFor='create-board-window_input'
                >
                    Board name
                </label>
                <input
                    id='create-board-window_input'
                    className='create-board-window_input'
                    type='text'
                    value={ boardName }
                    placeholder='Board name'
                    autoFocus
                    maxLength={ 40 }
                    onChange={ (event) => setBoardName(event.target.value) }
                />
                <div className='create-board-window_actions'>
                    <button
                        type='button'
                        className='create-board-window_cancel'
                        onClick={ handleCancel }
                    >
                        Cancel
                    </button>
                    <button
                        type='submit'
                        className='create-board-window_accept'
                        disabled={ !trimmedName }
                    >
                        Accept
                    </button>
                </div>
            </form>
        </div>
    );
}

export default CreateBoardWindow;
