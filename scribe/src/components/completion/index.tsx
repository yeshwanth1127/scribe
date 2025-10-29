import { useCompletion } from "@/hooks";
import { Input } from "./Input";
import { AudioGroup } from "./AudioGroup";
import { MediaGroup } from "./MediaGroup";
import { useSystemAudioType } from "@/hooks";

export const Completion = ({ 
  isHidden, 
  systemAudio 
}: { 
  isHidden: boolean;
  systemAudio?: useSystemAudioType;
}) => {
  const completion = useCompletion();

  return (
    <div className="flex flex-1 items-center gap-2">
      {/* Left spacer reserved for logo */}
      <div className="w-12 shrink-0" />
      {/* Input expands */}
      <div className="flex-1">
        <Input {...completion} isHidden={isHidden} />
      </div>
      {/* Grouped buttons on the right side of input */}
      <AudioGroup completion={completion} systemAudio={systemAudio} />
      <MediaGroup completion={completion} />
    </div>
  );
};
