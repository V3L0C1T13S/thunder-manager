import { useEffect, useState } from "react";
import { Box, Container, Grid, Pagination, Skeleton, Stack, Typography } from "@mui/material";
import { ListResponse } from "../utils/community";
import { ListCommunities } from "../utils";
import "../App.css";
import { useNavigate } from "react-router-dom";
import ActionAreaCard from "../components/ActionAreaCard";

export default function Browse() {
    const [communities, setCommunities] = useState<ListResponse | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        async function getCommunities() {
            try {
                const response = await ListCommunities();
                setCommunities(response);
            } catch (e) {
                console.error("error fetching communities:", e);
            }
        }

        if (!communities) getCommunities();
    })

    return (<Container>
        <Box textAlign="center" alignItems="center" alignContent="center" alignSelf="center">
            <Typography variant="h4">Browse</Typography>
            <br />
            <Grid container spacing={2} >
                {communities?.results ? communities.results.map((community) => (
                    <ActionAreaCard
                        key={community.identifier}
                        title={community.name}
                        content="Description"
                        height={140}
                        onClick={() => {
                            navigate(`/community/${community.identifier}`);
                        }}
                    />
                )) : <Skeleton variant="rectangular" width={210} height={118} />}
            </Grid>
            <br />
            <Stack spacing={2} alignItems="center">
                <Pagination count={10} showFirstButton showLastButton />
            </Stack>
        </Box>
    </Container>)
}