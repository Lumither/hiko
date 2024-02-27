'use client';

import { Divider, Navbar, NavbarContent } from '@nextui-org/react';
import { Button } from '@nextui-org/button';
import Link from 'next/link';
import Sidebar from '@/app/sidebar';
import Overview from '@/app/pages/overview';
import Tasks from '@/app/pages/tasks';
import { JSX, useState } from 'react';

export default function Home() {
    // const router = useRouter();

    let contentComponents: { [key: string]: JSX.Element } = {
        'overview': <Overview />,
        'tasks': <Tasks />
    };

    const [ selectedKey, setSelectedKey ] = useState(new Set([ 'overview' ]));


    return (
        <div className={ 'flex flex-col overflow-hidden h-screen' }>

            <Navbar position={ 'sticky' } maxWidth={ 'full' } className={ '' } isBlurred={ true }>
                {/*bg-[#212035]*/ }

                <NavbarContent justify={ 'start' }>
                    <p className={ 'text-xl' }>Hiko Control Panel</p>
                </NavbarContent>

                <NavbarContent justify={ 'end' }>
                    <Button as={ Link } href={ 'settings' } color={ 'default' } variant={ 'solid' }>
                        Settings
                    </Button>
                </NavbarContent>
            </Navbar>

            <div className={ 'flex flex-auto overflow-auto' }>

                {/* sidebar controller*/ }
                <div className={ 'flex-none w-2/12 ml-8 mt-4' }>
                    <Sidebar selectedKey={ selectedKey } setSelectedKey={ setSelectedKey } />
                </div>

                <Divider orientation={ 'vertical' } className={ 'ml-8 mr-8' } />

                {/* content loader */ }
                <div className={ 'flex-1 mr-8 overflow-auto mt-4' }>

                    { contentComponents[selectedKey.keys().next().value as string] }

                </div>
            </div>
        </div>
    );

}

