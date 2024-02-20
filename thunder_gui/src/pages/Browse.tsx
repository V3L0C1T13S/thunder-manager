import { useState } from "react";
import { Box, Container, Grid, Pagination, Skeleton, Stack, Typography } from "@mui/material";
import { ListResponse } from "../utils/community";
import { ListCommunities } from "../utils";
import "../App.css";
import { useNavigate, useParams } from "react-router-dom";
import ActionAreaCard from "../components/ActionAreaCard";

export default function Browse() {
    const [communities, setCommunities] = useState<ListResponse>({});
    const navigate = useNavigate();

    if (!communities.results) {
        ListCommunities()
            .then((response) => setCommunities(response))
            .catch((e) => console.error("error fetching communities:", e));
    }

    return (<Container>
        <Box textAlign="center" alignItems="center" alignContent="center" alignSelf="center">
            <Typography variant="h4">Browse</Typography>
            <br />
            <Grid container spacing={2} >
                {communities.results ? communities.results.map((community) => (
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