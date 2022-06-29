type CopyToClipboardProps = {
  copied: boolean;
  onClick: () => void;
  text: string;
};

const CopyToClipboard = ({ copied, onClick, text }: CopyToClipboardProps) => {
  const title = copied ? "Copied!" : "Copy to clipboard";
  return (
    <button
      title={title}
      class="absolute right-6 hover:text-fuchsia-400 transition"
      onClick={() => {
        navigator.clipboard.writeText(text);
        onClick();
      }}
    >
      {copied ? (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-6 w-6 text-green-300"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M5 13l4 4L19 7"
          />
        </svg>
      ) : (
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-6 w-6"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="2"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2"
          />
        </svg>
      )}
    </button>
  );
};

export default CopyToClipboard;
