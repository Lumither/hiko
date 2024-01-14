'use client';

import { Card, CardBody, Listbox, ListboxItem } from '@nextui-org/react';
import { Dispatch, SetStateAction } from 'react';

type Interface = {
    selectedKey: Set<string>,
    setSelectedKey: Dispatch<SetStateAction<Set<string>>>
}
export default function Sidebar(props: Interface) {

    return (
        <div className={ 'flex flex-col gap-2' }>
            <Card>
                <CardBody>
                    <Listbox
                        variant={ 'flat' }
                        selectionMode={ 'single' }
                        disallowEmptySelection
                        selectedKeys={ props.selectedKey }
                        onSelectionChange={ (keys) => props.setSelectedKey(keys as Set<string>) }
                    >
                        <ListboxItem key="overview">Overview</ListboxItem>
                        <ListboxItem key="tasks">Tasks</ListboxItem>
                    </Listbox>
                </CardBody>
            </Card>

        </div>
    );
}