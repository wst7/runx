import SettingSvg from "@/assets/setting.svg?react";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";

export type Menu = {
  icon: React.FC<React.SVGProps<SVGSVGElement>>;
  title: string;
  onClick?: () => void;
};

export default function Sidebar(props: { menus: Menu[], onSettingClick?: () => void}) {
  const { menus, onSettingClick } = props;


  return (
    <div className="w-[40px] h-full bg-[#1e1e1e] text-white border-r border-[#282c34] flex flex-col justify-between">
      <div className="flex flex-col">
        {menus.map((menu) => (
          <div
            key={menu.title}
            className="flex justify-center items-center h-[40px] cursor-pointer hover:bg-[#282c34]"
            onClick={menu.onClick}
          >
            <Tooltip>
              <TooltipTrigger>
                <menu.icon
                  style={{
                    width: "20px",
                    height: "20px",
                    fill: "currentColor",
                  }}
                />
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>{menu.title}</p>
              </TooltipContent>
            </Tooltip>
          </div>
        ))}
      </div>
      <div className="flex justify-center items-center h-[40px] cursor-pointer hover:bg-[#282c34]">
        <Tooltip>
          <TooltipTrigger>
            <SettingSvg
              style={{ width: "20px", height: "20px", fill: "currentColor" }}
              onClick={onSettingClick}
            />
          </TooltipTrigger>
          <TooltipContent side="right">
            <p>{"setting"}</p>
          </TooltipContent>
        </Tooltip>
      </div>
    </div>
  );
}
