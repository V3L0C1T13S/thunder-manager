import { Paper, Table, TableBody, TableCell, TableContainer, TableRow } from "@mui/material"

export type Row = {
    name: string
    value?: string | number
}

export type InfoRowProps = {
    rows: Row[],
}

export default function InfoRow(props: InfoRowProps) {
    return <TableContainer component={Paper}>
        <Table sx={{ minWidth: 345 }}>
            <TableBody>
                {props.rows.map((row) => <TableRow>
                    <TableCell>{row.name}</TableCell>
                    <TableCell>{row.value}</TableCell>
                </TableRow>)}
            </TableBody>
        </Table>
    </TableContainer>
}