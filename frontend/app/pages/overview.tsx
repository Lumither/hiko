import { Card, CardBody } from '@nextui-org/react';
import { PieChart } from '@mui/x-charts/PieChart';
import MUIDarkLoader from '@/app/muiDarkLoader';

export default function Overview() {


    return (
        <>
            <Card
                isBlurred={ true }
                shadow={ 'sm' }
                className={ 'border-none bg-background/60 dark:bg-default-100/50 max-w-[610px]' }
            >
                <CardBody
                >
                    <div className="grid grid-cols-6 md:grid-cols-12 gap-6 md:gap-4 items-center justify-center">
                        <div className="relative col-span-6 md:col-span-4">
                            <MUIDarkLoader>
                                <PieChart
                                    series={ [
                                        {
                                            data: [
                                                { id: 0, value: 10, label: 'series A' },
                                                { id: 1, value: 15, label: 'series B' },
                                                { id: 2, value: 20, label: 'series C' }
                                            ],
                                            innerRadius: 30,
                                            outerRadius: 100,
                                            paddingAngle: 5,
                                            cornerRadius: 5,
                                            startAngle: -90,
                                            endAngle: 180,
                                            cx: 150,
                                            cy: 150
                                        }
                                    ] }
                                    width={ 400 }
                                    height={ 600 }
                                />
                            </MUIDarkLoader>
                        </div>
                    </div>
                </CardBody>
            </Card>
        </>
    );
}
