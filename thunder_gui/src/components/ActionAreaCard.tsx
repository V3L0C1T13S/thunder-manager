import { Card, CardActionArea, CardContent, CardMedia, Typography } from '@mui/material';
import { placeholderImages } from '../utils/assets';

export type ActionAreaCardProps = {
    title: string,
    content: string,
    image?: string,
    alt?: string,
    height?: number,
    onClick?: () => void,
}

export default function ActionAreaCard(props: ActionAreaCardProps) {
    const placeholderImage = placeholderImages[Math.floor(Math.random() * placeholderImages.length)];

    return (
        <Card sx={{ maxWidth: 345 }}>
            <CardActionArea
                onClick={() => props.onClick?.()}
            >
                <CardMedia
                    component="img"
                    height={props.height ?? 140}
                    image={props.image ?? placeholderImage}
                    alt={props.alt}
                />
                <CardContent>
                    <Typography gutterBottom variant="h5" component="div">
                        {props.title}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                        {props.content}
                    </Typography>
                </CardContent>
            </CardActionArea>
        </Card>
    );
}