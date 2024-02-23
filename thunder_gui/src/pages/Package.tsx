import { Box, Button, Card, CardContent, Container, Skeleton, Typography } from "@mui/material";
import { FetchPackage } from "../utils";
import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import { ThunderstorePackage } from "../utils/package";
import { copyToClipboard } from "../utils/clipboard";
import InfoRow from "../components/InfoRow";

export default function Package() {
    const { name, namespace } = useParams();
    const [packageData, setPackageData] = useState<ThunderstorePackage | null>(null);

    useEffect(() => {
        async function fetchPackageData() {
            try {
                if (!name || !namespace) {
                    console.error("no name or namespace", name, namespace);
                    return;
                }

                const data = await FetchPackage(name, namespace);

                console.log(data);

                setPackageData(data);
            } catch (e) {
                console.error("couldn't fetch pacakge:", e);
            }
        }

        if (!packageData) fetchPackageData();
    });

    const tableRows = [{
        name: "Last updated",
        value: packageData?.date_updated,
    }, {
        name: "Total downloads",
        value: packageData?.total_downloads,
    }, {
        name: "Rating",
        value: packageData?.rating_score,
    }, {
        name: "Dependency string",
        value: packageData?.full_name,
    }]

    return <Container maxWidth="sm">
        {packageData ? <Box>
            <Card sx={{ minWidth: 345, maxWidth: 650 }}>
                <CardContent>
                    <Typography gutterBottom variant="h5" component="div">{packageData.name}</Typography>
                    <Typography variant="body2" color="text.secondary">{packageData.latest.description}</Typography>
                    <Typography variant="body1">By&nbsp;</Typography>
                    <Typography variant="body1" color="turquoise">{packageData.owner}</Typography>
                    <InfoRow rows={tableRows} />
                    <Button>Add</Button>
                    {packageData.package_url ? <Button
                        onClick={() => copyToClipboard(packageData.package_url ?? "")}
                    >Copy link</Button> : <></>}
                </CardContent>
            </Card>
        </Box> : <Skeleton variant="rectangular" height={250} width={345} />}
    </Container>
}