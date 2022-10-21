export const ErrorPath: React.FC<{ status: number; details: String }> = ({ status, details }) => {
  return (
    <section className="error-page-wrapper">
      <div className="error-status">{status}</div>
      <div className="error-details">{details}</div>
    </section>
  );
};
