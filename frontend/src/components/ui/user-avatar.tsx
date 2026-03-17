"use client"

import * as React from "react"

import { cn } from "@/lib/utils"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"

type UserAvatarProps = Omit<React.ComponentProps<typeof Avatar>, "children"> & {
  src?: string | null
  alt?: string
  initials?: string
  imageClassName?: string
  fallbackClassName?: string
}

export function UserAvatar({
  src,
  alt,
  initials,
  className,
  imageClassName,
  fallbackClassName,
  ...props
}: UserAvatarProps) {
  const safeInitials =
    (initials ?? "")
      .trim()
      .split(/\s+/)
      .join("")
      .slice(0, 2)
      .toUpperCase() || "?"

  return (
    <Avatar className={cn("size-9", className)} {...props}>
      {src ? (
        <AvatarImage
          src={src}
          alt={alt ?? ""}
          className={cn(imageClassName)}
        />
      ) : null}
      <AvatarFallback
        className={cn(
          "bg-muted text-muted-foreground text-xs font-medium",
          fallbackClassName
        )}
      >
        {safeInitials}
      </AvatarFallback>
    </Avatar>
  )
}

