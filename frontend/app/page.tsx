import Overview from "./overview/page";
import Sidebar from "./sidebar";

export default function Home() {
    return (
        <>
            <Sidebar>
                {/* todo: sidebar switch */}
                <Overview/>
            </Sidebar>
        </>
    )
}
