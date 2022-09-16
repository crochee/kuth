import { useState } from 'react';
import './index.css';
import { ButtonClose } from '../button';

function Offcanvas(props) {
    return <div className="offcanvas" tabindex="-1">
        <div className="offcanvas-header">
            <h5 className="offcanvas-title">Kuth</h5>
            <ButtonClose onClick={props.onClick} />
        </div>
        <div className="offcanvas-body">
            Content for the offcanvas goes here.
        </div>
    </div>
}

function Service(props) {
    return <svg xmlns="http://www.w3.org/2000/svg" className="service" onMouseEnter={props.onClick} width="24" height="24" fill="currentColor" viewBox="0 0 448 512">
        <path d="M0 96C0 78.3 14.3 64 32 64H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32C14.3 128 0 113.7 0 96zM0 256c0-17.7 14.3-32 32-32H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H32c-17.7 0-32-14.3-32-32zM448 416c0 17.7-14.3 32-32 32H32c-17.7 0-32-14.3-32-32s14.3-32 32-32H416c17.7 0 32 14.3 32 32z" />
    </svg>
}

export default function SideBar() {
    let [visible, switchVisible] = useState(false)
    return <div className="sidebar">
        <Service onClick={() => {
            switchVisible(true)
        }} />
        <p className="sidebar-line"></p>
        {
            visible && <Offcanvas onClick={() => {
                switchVisible(false)
            }} />
        }
    </div>
}