import Link from 'next/link'
import React, {ReactNode} from 'react'

const Sidebar = ({children}: { children: ReactNode }) => {
    return (
        <div className="drawer lg:drawer-open">
            <input className="drawer-toggle"/>
            <div className="drawer-content flex flex-col items-center justify-center">
                {children}
            </div>
            <div className="drawer-side">
                {/* <label htmlFor="my-drawer-2" aria-label="close sidebar" className="drawer-overlay"></label> */}
                <ul className="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                    {/* Sidebar content here */}
                    <li><h1 className='text-4xl'>Hiko Dashboard</h1></li>
                    <li>
                        <Link className='text-xl' href={"/overview"}>Overview</Link>
                    </li>
                </ul>
            </div>
        </div>


    )
}

export default Sidebar
