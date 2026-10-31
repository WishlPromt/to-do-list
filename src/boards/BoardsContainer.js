import { useState } from 'react';
import CreateBoardWindow from './CreateBoardWindow';
import './styles/BoardsContainer.css';
import CreateBoardButton from './CreateBoardButton';

function renderBoards() {

}

function BoardsContainer() {
    const [boards, setBoards] = useState([]);
    const [isCreateBoardOpen, SetIsCreateBoardOpen] = useState(false);

    function handleAccept(boardName) {
        setBoards([...boards, boardName])
        console.log(boards);
        SetIsCreateBoardOpen(false);
    }

    const handleOpen = () => SetIsCreateBoardOpen(true);

    return (
        <>
            { boards.length <= 0 && (
                <CreateBoardButton onCreate={handleOpen} variant='button'/>
            )}

            { boards.length > 0 && (
                <div className='boards-container'>
                    <CreateBoardButton onCreate={handleOpen} variant='card'/>
                </div>
            )}

            { isCreateBoardOpen && (
                <CreateBoardWindow 
                    onAccept={ handleAccept }
                    onCancel={ () => SetIsCreateBoardOpen(false) }
                />
            )}
        </>
    );
}

export default BoardsContainer;
