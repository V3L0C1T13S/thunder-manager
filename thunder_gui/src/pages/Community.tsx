import { Box, Container, Grid, Skeleton, Typography } from "@mui/material";
import { useState } from "react";
import { useParams } from "react-router-dom"
import { PackageList } from "../utils/package";
import { GetCommunityPackages } from "../utils";
import ActionAreaCard from "../components/ActionAreaCard";

export default function Community() {
    const { id } = useParams();
    const [packages, setPackages] = useState<PackageList | null>(null);

    if (!id) return <Container>
        <Typography>Awkward... You didn't choose a community.</Typography>
    </Container>

    if (!packages) {
        GetCommunityPackages(id).then((list) => setPackages(list));
    }

    return <Container maxWidth="sm">
        <Box textAlign="center">
            <Typography variant="h4">{id}</Typography>
            <br />
            <Grid container spacing={2}>
                {packages ? packages.packages.map((card) => (
                    <ActionAreaCard
                        key={card.package_name}
                        title={card.package_name}
                        content={`${card.download_count} Downloads`}
                        image={card.image_src}
                    />
                )) : <Skeleton variant="rounded" width={500} height={100} />}
            </Grid>
        </Box>
    </Container>
}