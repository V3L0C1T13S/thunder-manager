import { Box, Container, Grid, Skeleton, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom"
import { PackageList } from "../utils/package";
import { GetCommunityPackages } from "../utils";
import ActionAreaCard from "../components/ActionAreaCard";

export default function Community() {
    const { id } = useParams();
    const [packages, setPackages] = useState<PackageList | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        async function fetchPackages() {
            try {
                if (!id) return;

                const response = await GetCommunityPackages(id)
                setPackages(response);
            } catch (e) {
                console.error("error fetching packages:", e);
            }
        }

        if (!packages) fetchPackages();
    });

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
                        categories={card.categories.map((category) => category.name)}
                        onClick={() => navigate(`/package/${card.namespace}/${card.package_name}`)}
                    />
                )) : <Skeleton variant="rounded" width={500} height={100} />}
            </Grid>
        </Box>
    </Container>
}