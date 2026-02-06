
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from "../../../ui/dialog";
import { Label } from "../../../ui/label";
import { Input } from "../../../ui/input";
import { Button } from "../../../ui/button";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTrackStore } from "@/Stores/TrackStore";

export interface DetailsRegionSetModalProps {
  open: boolean;
  regionSetId: string;
  onClose: () => void;
}

export function DetailsRegionSetModal({ regionSetId, open, onClose }: DetailsRegionSetModalProps) {
   
  
    // If region exists, pull the regionSet
    const regionSet = useRegionSetStore(state => state.regionSets.get(regionSetId));
    // If regionSet exists, pull the track
    const track = useTrackStore(state => 
      regionSet ? state.tracks.get(regionSet.trackId) : null
    );
  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="space-y-6">
        <DialogHeader>
          <DialogTitle>Region Set Details</DialogTitle>
        </DialogHeader>

        <div className="space-y-2">
          <Label htmlFor="track-id">Track ID</Label>
          <Input id="track-id" value={track?.trackId} readOnly />
        </div>

         <div className="space-y-2">
          <Label htmlFor="track-id">Region Set ID</Label>
          <Input id="track-id" value={regionSet?.id} readOnly />
        </div>

        <div className="space-y-2">
          <Label htmlFor="track-name">Name</Label>
          <Input id="track-name" value={regionSet?.name} readOnly />
        </div>

        

        {/* <RegionTable regions={regions.entries()} /> */}

        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            Exit
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

// 🔹 Extracted for cleanliness
// function RegionTable({ regions }: { regions: TrackRegionViewModel[] }) {
//   if (regions.length === 0) return <p className="text-muted-foreground">No regions available.</p>;

//   return (
//     <div className="space-y-2">
//       <Label>Regions</Label>
//       <Table>
//         <TableHeader>
//           <TableRow>
//             <TableHead>ID</TableHead>
//             <TableHead>Name</TableHead>
//           </TableRow>
//         </TableHeader>
//         <TableBody>
//           {regions.map((region) => (
//             <TableRow key={region.regionId}>
//               <TableCell>{region.regionId}</TableCell>
//               <TableCell>{region.name}</TableCell>
//             </TableRow>
//           ))}
//         </TableBody>
//       </Table>
//     </div>
//   );
// }
